import logging
import os
from typing import List

from dotenv import load_dotenv
from pymongo.mongo_client import MongoClient

load_dotenv()

formatter = logging.Formatter(
    '{"timestamp": "%(asctime)s", "module": "%(module)s", "message": "%(message)s", "severity": "%(levelname)s"'
)
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("db")

class VectorDB:
    def __init__(self, conn_string: str):
        self.client = self._get_client(conn_string)
        self.embedding_db = self.client.note.embeddings

    def add_emb(self, _id: int, embeddings):
        embeddings = embeddings.tolist()
        self.embedding_db.insert_one(dict(_id=_id, vec=embeddings))

    def remove_emb(self, _id: int):
        self.embedding_db.delete_one({"_id": _id})

    def get_embds(self):
        ids, embds = [], []
        for item in self.embedding_db.find():
            ids.append(item["_id"])
            embds.append(item["vec"])
        return ids, embds

    def _get_client(self, conn_stirng: str):
        try:
            client = MongoClient(conn_stirng)
            client.admin.command("ping")
            logger.info("connection to db established")
            return client
        except Exception as e:
            logger.info(f"db doesn't response with error: {e}")


if __name__ == "__main__":
    conn_string = os.environ.get("VECTOR_DB_CONN_STRING")
    vec = [8] * 100
    db = VectorDB(conn_string)
    db.add_emb(0, vec)
    db.add_emb(1, vec)
    db.add_emb(2, vec)
    get_all = db.get_embds()
    logger.info(get_all)

    for item in db.client.note.embeddings.find():
        logger.info(item)
        db.remove_emb(item["_id"])
