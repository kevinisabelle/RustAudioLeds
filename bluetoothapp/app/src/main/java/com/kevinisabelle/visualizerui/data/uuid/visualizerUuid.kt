package com.kevinisabelle.visualizerui.data.uuid

import java.util.UUID

/** Base is 3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63. Offset 1 → Smooth Size, etc. */
fun visualizerUuid(offset: Int): UUID =
    UUID.fromString(String.format("3E0E%04X-7C7A-47B0-9FD5-1FC3044C3E63", offset))