import logging
from typing import List

import numpy as np
from sentence_transformers import SentenceTransformer

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("embeddings")

# Load the model
model = SentenceTransformer("obrizum/all-MiniLM-L6-v2")

def embed(sentences: List[str]) -> List[List[float]]:
    embds = model.encode(sentences)[0]
    logger.info(embds)
    return embds


def cosine_sim(query: List[List[float]], source: List[List[float]]):
    query_arr = np.array(query).reshape(-1, 1)  # reshape for dot product
    source_arr = np.array(source)
    logger.info(source_arr.shape)

    # normalize
    source_arr = source_arr / np.linalg.norm(source_arr, axis=1, keepdims=True)
    query_arr = query_arr / np.linalg.norm(query_arr, axis=0, keepdims=True)

    sim = np.dot(source_arr, query_arr).reshape(-1)
    max_indices = np.argsort(sim, axis=0)[-5:][::-1]

    # return top 5
    return max_indices, sim[max_indices]
