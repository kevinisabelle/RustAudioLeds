DOMAIN = "audioleds"
PLATFORMS: list[str] = ["light"]

CONF_HOST = "host"
CONF_PORT = "port"
CONF_TOKEN = "token"
CONF_VERIFY_SSL = "verify_ssl"

DEFAULT_PORT = 3000
DEFAULT_VERIFY_SSL = False  # device uses self-signed cert
UPDATE_INTERVAL_SECONDS = 15

DATA_API = "api"
DATA_COORDINATOR = "coordinator"

SERVICE_SEND_COMMAND = "send_command"
SERVICE_CREATE_PRESET = "create_preset"
SERVICE_ACTIVATE_PRESET = "activate_preset"
SERVICE_DELETE_PRESET = "delete_preset"
SERVICE_SET_COLORS = "set_colors"

ATTR_PRESET_ID = "preset_id"
