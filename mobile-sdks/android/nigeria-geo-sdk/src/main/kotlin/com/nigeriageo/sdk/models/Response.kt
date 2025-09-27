package com.nigeriageo.sdk.models

import kotlinx.serialization.Serializable

@Serializable
data class PaginatedResponse<T>(
    val data: List<T>,
    val pagination: Pagination
)

@Serializable
data class Pagination(
    val page: Int,
    val limit: Int,
    val total: Int,
    @kotlinx.serialization.SerialName("total_pages") val totalPages: Int,
    @kotlinx.serialization.SerialName("has_next") val hasNext: Boolean,
    @kotlinx.serialization.SerialName("has_prev") val hasPrev: Boolean
)

@Serializable
data class SearchResult(
    val states: List<NigerianState>,
    val lgas: List<LGA>,
    val wards: List<Ward>,
    @kotlinx.serialization.SerialName("postal_codes") val postalCodes: List<PostalCode>
)