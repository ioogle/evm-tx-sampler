import os
import tomli

class Config:
    def __init__(self, filename = '../config/production.toml'):
        base_path = os.path.dirname(__file__)
        file_path = os.path.join(base_path, filename)

        with open(file_path, 'rb') as f:
            self._config = tomli.load(f)

    @property
    def app(self):
        return self._config.get('app', {})

    @property
    def backend_url(self):
        return self.app.get('backend_url', '')

config = Config()