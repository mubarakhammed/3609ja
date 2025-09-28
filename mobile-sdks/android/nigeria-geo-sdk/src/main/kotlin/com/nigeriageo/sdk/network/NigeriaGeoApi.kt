package com.nigeriageo.sdk.network

import com.nigeriageo.sdk.models.*
import retrofit2.http.*

interface NigeriaGeoApi {
    
    @GET("api/v1/states")
    suspend fun getStates(
        @Query("page") page: Int = 1,
        @Query("limit") limit: Int = 20
    ): PaginatedResponse<NigerianState>
    
    @GET("api/v1/states/{id}")
    suspend fun getStateById(@Path("id") stateId: String): NigerianState
    
    @GET("api/v1/states/{id}/lgas")
    suspend fun getLGAs(
        @Path("id") stateId: String,
        @Query("page") page: Int = 1,
        @Query("limit") limit: Int = 20
    ): PaginatedResponse<LGA>
    
    @GET("api/v1/lgas/{id}")
    suspend fun getLGAById(@Path("id") lgaId: String): LGA
    
    @GET("api/v1/lgas/{id}/wards")
    suspend fun getWards(
        @Path("id") lgaId: String,
        @Query("page") page: Int = 1,
        @Query("limit") limit: Int = 20
    ): PaginatedResponse<Ward>
    
    @GET("api/v1/wards/{id}")
    suspend fun getWardById(@Path("id") wardId: String): Ward
    
    @GET("api/v1/wards/{id}/postal-codes")
    suspend fun getPostalCodes(
        @Path("id") wardId: String,
        @Query("page") page: Int = 1,
        @Query("limit") limit: Int = 20
    ): PaginatedResponse<PostalCode>
    
    @GET("api/v1/search")
    suspend fun search(@Query("query") query: String): SearchResult
}