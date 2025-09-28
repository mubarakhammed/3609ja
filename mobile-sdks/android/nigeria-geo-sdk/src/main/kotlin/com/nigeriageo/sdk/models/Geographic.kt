package com.nigeriageo.sdk.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class NigerianState(
    val id: String,
    val name: String,
    val code: String,
    @SerialName("created_at") val createdAt: String,
    @SerialName("updated_at") val updatedAt: String
)

@Serializable
data class LGA(
    val id: String,
    @SerialName("state_id") val stateId: String,
    val name: String,
    val code: String,
    @SerialName("created_at") val createdAt: String,
    @SerialName("updated_at") val updatedAt: String
)

@Serializable
data class Ward(
    val id: String,
    @SerialName("lga_id") val lgaId: String,
    val name: String,
    val code: String,
    @SerialName("created_at") val createdAt: String,
    @SerialName("updated_at") val updatedAt: String
)

@Serializable
data class PostalCode(
    val id: String,
    @SerialName("ward_id") val wardId: String?,
    val code: String,
    val area: String,
    @SerialName("created_at") val createdAt: String,
    @SerialName("updated_at") val updatedAt: String
)