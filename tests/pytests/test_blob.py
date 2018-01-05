from rain.client import blob, RainException
import pytest
import json
import pickle


def test_blob_construction(fake_session):
    with fake_session as session:
        b1 = blob("abc")
        assert b1.session == session

        b2 = blob(b"xyz")
        assert b1.session == session
        assert b1.id != b2.id

        obj = [1, {'a': (4, 5)}]
        b3 = blob(obj, encode='pickle')
        assert pickle.loads(b3.data) == obj

        b4 = blob(obj, encode='json')
        assert json.loads(b4.data)

        txt = "asžčďďŠ"
        b5 = blob(txt, encode='text:latin2')
        assert b5.data.decode('latin2') == txt

        with pytest.raises(RainException):
            blob(123)
