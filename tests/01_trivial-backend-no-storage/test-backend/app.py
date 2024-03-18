import logging

from fastapi import FastAPI


logger = logging.getLogger(__name__)


def create_app():
    app = FastAPI()

    @app.get('/api/v1/healthz')
    async def healthz():
        return {'status': True}
    return app


app = create_app()
