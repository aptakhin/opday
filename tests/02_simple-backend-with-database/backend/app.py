import logging
import os

import asyncpg
from fastapi import FastAPI


logger = logging.getLogger(__name__)


def create_app():
    app = FastAPI()

    @app.get('/api/v1/healthz')
    async def healthz():
        return {'status': True}

    @app.get('/api/v1/make-database-call')
    async def make_database_call():
        db_dsn = os.environ["DATABASE_DSN"]
        con = await asyncpg.connect(dsn=db_dsn)
        types = await con.fetch('SELECT * FROM pg_type')
        print(types)
        return {'status': True}

    return app


app = create_app()
