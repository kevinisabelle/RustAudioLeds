from __future__ import annotations

from datetime import timedelta
from typing import Any, Iterable
import logging

from homeassistant.core import HomeAssistant
from homeassistant.helpers.update_coordinator import DataUpdateCoordinator, UpdateFailed

from .api import AudioLedsApi, AudioLedsApiError
from .const import UPDATE_INTERVAL_SECONDS, DOMAIN


class AudioLedsDataUpdateCoordinator(DataUpdateCoordinator[dict[str, Any]]):
    """Coordinator to poll AudioLeds device.

    Data structure exposed via ``self.data``:
    {
        "info": <dict>,              # Raw data from /api/v1/info
        "presets": [<dict>, ...],    # List from /api/v1/presets
        "active_preset": <dict|None> # Best-effort detection of active preset
    }
    """

    def __init__(self, hass: HomeAssistant, api: AudioLedsApi) -> None:
        super().__init__(
            hass,
            logging.getLogger(__name__),
            name=DOMAIN,
            update_interval=timedelta(seconds=UPDATE_INTERVAL_SECONDS),
        )
        self.api = api

    @staticmethod
    def _find_active_preset(presets: Iterable[dict[str, Any]]) -> dict[str, Any] | None:
        """Heuristically determine the active preset from list.

        Looks for common truthy flags or a matching state marker.
        """
        for p in presets:
            if not isinstance(p, dict):
                continue
            if any(
                p.get(flag) in (True, 1, "true", "True")
                for flag in ("active", "is_active", "current", "enabled")
            ):
                return p
        # Secondary heuristic: brightness > 0 could indicate active, but skip for now.
        return None

    async def _async_update_data(self) -> dict[str, Any]:
        try:
            info = await self.api.async_get_info()
        except AudioLedsApiError as err:
            raise UpdateFailed(f"Failed fetching info: {err}") from err

        try:
            presets = await self.api.async_list_presets()
            if not isinstance(presets, list):  # Normalize unexpected structure
                presets = []
        except AudioLedsApiError:
            presets = []

        active = self._find_active_preset(presets)
        return {"info": info, "presets": presets, "active_preset": active}
