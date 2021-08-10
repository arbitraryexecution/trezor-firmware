from common import *
from apps.common import cbor
from trezor.crypto import hashlib

if not utils.BITCOIN_ONLY:
    from apps.cardano.helpers.lazy_cbor_collection import LazyCborDict, LazyCborList
    from apps.cardano.helpers.cbor_hash_builder import CborHashBuilder


@unittest.skipUnless(not utils.BITCOIN_ONLY, "altcoin")
class TestCborHashBuilder(unittest.TestCase):
    def test_get_hash(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(3))

        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(1))
        cbor_hash_builder.add_item(("input", 0))
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.add_lazy_collection_at_key(1, LazyCborList(1))
        cbor_hash_builder.add_lazy_collection(LazyCborList(2))
        cbor_hash_builder.add_item("output_address")
        cbor_hash_builder.add_lazy_collection(LazyCborList(2))
        cbor_hash_builder.add_item(1)
        cbor_hash_builder.add_lazy_collection(LazyCborDict(1))
        cbor_hash_builder.add_lazy_collection_at_key(b"policy_id", LazyCborDict(1))
        cbor_hash_builder.add_item((b"asset_name_bytes", 2))
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.add_item((2, 42))
        cbor_hash_builder.finish_current_lazy_collection()

        hash = cbor_hash_builder.get_hash()

        expected_hash = hashlib.blake2b(
            data=cbor.encode(
                {
                    0: [["input", 0]],
                    1: [
                        [
                            "output_address",
                            [1, {b"policy_id": {b"asset_name_bytes": 2}}],
                        ]
                    ],
                    2: 42,
                }
            ),
            outlen=32,
        ).digest()

        self.assertEqual(
            hash,
            expected_hash,
        )

    def test_get_hash_on_unfinished(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(3))
        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(1))

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.get_hash()

    def test_add_lazy_collection_without_key_to_dict(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(3))

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.add_lazy_collection(LazyCborList(1))

    def test_add_lazy_collection_at_key_to_list(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(3))
        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(1))

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.add_lazy_collection_at_key(1, LazyCborList(1))

    def test_add_item_when_already_finished(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(1))
        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(1))
        cbor_hash_builder.add_item(("input", 0))
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.add_item(("input", 1))

    def test_finish_current_lazy_collection_when_already_finished(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(1))
        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(1))
        cbor_hash_builder.add_item(("input", 0))
        cbor_hash_builder.finish_current_lazy_collection()
        cbor_hash_builder.finish_current_lazy_collection()

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.finish_current_lazy_collection()

    def test_finish_current_lazy_collection_when_collection_not_filled(self):
        cbor_hash_builder = CborHashBuilder(hashlib.blake2b(outlen=32), LazyCborDict(1))
        cbor_hash_builder.add_lazy_collection_at_key(0, LazyCborList(2))
        cbor_hash_builder.add_item(("input", 0))

        with self.assertRaises(RuntimeError):
            cbor_hash_builder.finish_current_lazy_collection()


if __name__ == "__main__":
    unittest.main()
