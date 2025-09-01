from __future__ import annotations

import asyncio
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

import aiohttp


class AudioLedsApiError(Exception):
    """Base exception for API errors."""


class AudioLedsAuthError(AudioLedsApiError):
    """Authentication / authorization errors."""


@dataclass
class AudioLedsConfig:
    host: str
    port: int
    token: str
    verify_ssl: bool = False

    @property
    def base_url(self) -> str:
        # Device uses HTTPS per Postman collection example
        return f"https://{self.host}:{self.port}".rstrip(":")


class AudioLedsApi:
    def __init__(self, session: aiohttp.ClientSession, config: AudioLedsConfig) -> None:
        self._session = session
        self._config = config
        self._lock = asyncio.Lock()

    async def _request(self, method: str, path: str, json: Any | None = None) -> Any:
        url = f"{self._config.base_url}{path}"
        headers = {"Authorization": f"Bearer {self._config.token}", "Accept": "application/json"}
        if json is not None:
            headers["Content-Type"] = "application/json"
        ssl = None if self._config.verify_ssl else False
        try:
            async with self._lock:  # serialize to be gentle (optional)
                async with self._session.request(method, url, headers=headers, json=json, ssl=ssl) as resp:
                    if resp.status in (401, 403):
                        raise AudioLedsAuthError(f"Auth failed ({resp.status})")
                    if resp.status >= 400:
                        text = await resp.text()
                        raise AudioLedsApiError(f"HTTP {resp.status} for {url}: {text}")
                    if resp.content_type == "application/json":
                        return await resp.json()
                    return await resp.text()
        except aiohttp.ClientError as err:
            raise AudioLedsApiError(f"Network error talking to AudioLeds: {err}") from err

    async def async_get_info(self) -> Dict[str, Any]:
        return await self._request("GET", "/api/v1/info")

    async def async_post_command(self, partial: Dict[str, Any]) -> Dict[str, Any]:
        return await self._request("POST", "/api/v1/command", json=partial)

    async def async_list_presets(self) -> List[Dict[str, Any]]:
        data = await self._request("GET", "/api/v1/presets")
        # Accept either list or wrapped structure
        if isinstance(data, dict) and "presets" in data:
            return data["presets"]
        if isinstance(data, list):
            return data
        return []

    async def async_get_preset(self, preset_id: int | str) -> Dict[str, Any]:
        return await self._request("GET", f"/api/v1/presets/{preset_id}")

    async def async_delete_preset(self, preset_id: int | str) -> None:
        await self._request("DELETE", f"/api/v1/presets/{preset_id}")

    async def async_activate_preset(self, preset_id: int | str) -> Any:
        return await self._request("POST", f"/api/v1/presets/{preset_id}/activate")

    async def async_create_preset(self, data: Dict[str, Any]) -> Any:
        return await self._request("POST", "/api/v1/presets", json=data)

