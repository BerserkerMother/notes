import logging
import os

from dotenv import load_dotenv
from pymongo.mongo_client import MongoClient

load_dotenv()

formatter = logging.Formatter(
    '{"timestamp": "%(asctime)s", "module": "%(module)s", "message": "%(message)s", "severity": "%(levelname)s"'
)
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("db")


def get_client(conn_stirng: str):
    try:
        client = MongoClient(conn_stirng)
        client.admin.command("ping")
        logger.info("connection to db established")
        return client
    except Exception as e:
        logger.info(f"db doesn't response with error: {e}")


if __name__ == "__main__":
    logger.info(os.environ.get("VECTOR_DB_CONN_STRING"))
    client = get_client(os.environ.get("VECTOR_DB_CONN_STRING"))
    logging.info(client)
