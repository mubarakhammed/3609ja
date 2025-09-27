package com.nigeriageo.sdk

import com.jakewharton.retrofit2.converter.kotlinx.serialization.asConverterFactory
import com.nigeriageo.sdk.models.*
import com.nigeriageo.sdk.network.NigeriaGeoApi
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import java.util.concurrent.TimeUnit

/**
 * Main SDK client for Nigeria Geo API
 * Zero configuration required - works out of the box!
 */
class NigeriaGeoSDK private constructor(
    private val api: NigeriaGeoApi,
    private val baseUrl: String = DEFAULT_BASE_URL
) {
    
    companion object {
        private const val DEFAULT_BASE_URL = "http://localhost:3000/"
        
        @Volatile
        private var instance: NigeriaGeoSDK? = null
        
        /**
         * Get SDK instance (creates if needed)
         * Zero configuration required!
         */
        fun getInstance(
            baseUrl: String = DEFAULT_BASE_URL,
            enableLogging: Boolean = false
        ): NigeriaGeoSDK {
            return instance ?: synchronized(this) {
                instance ?: createInstance(baseUrl, enableLogging).also { instance = it }
            }
        }
        
        private fun createInstance(baseUrl: String, enableLogging: Boolean): NigeriaGeoSDK {
            val json = Json {
                ignoreUnknownKeys = true
                coerceInputValues = true
            }
            
            val okHttpClient = OkHttpClient.Builder()
                .connectTimeout(30, TimeUnit.SECONDS)
                .readTimeout(30, TimeUnit.SECONDS)
                .writeTimeout(30, TimeUnit.SECONDS)
                .apply {
                    if (enableLogging) {
                        addInterceptor(
                            HttpLoggingInterceptor().apply {
                                level = HttpLoggingInterceptor.Level.BODY
                            }
                        )
                    }
                }
                .build()
            
            val retrofit = Retrofit.Builder()
                .baseUrl(baseUrl)
                .client(okHttpClient)
                .addConverterFactory(json.asConverterFactory("application/json".toMediaType()))
                .build()
            
            val api = retrofit.create(NigeriaGeoApi::class.java)
            return NigeriaGeoSDK(api, baseUrl)
        }
    }
    
    // States API
    suspend fun getStates(page: Int = 1, limit: Int = 20): PaginatedResponse<NigerianState> {
        return withContext(Dispatchers.IO) {
            api.getStates(page, limit)
        }
    }
    
    suspend fun getStateById(stateId: String): NigerianState {
        return withContext(Dispatchers.IO) {
            api.getStateById(stateId)
        }
    }
    
    // LGAs API
    suspend fun getLGAs(stateId: String, page: Int = 1, limit: Int = 20): PaginatedResponse<LGA> {
        return withContext(Dispatchers.IO) {
            api.getLGAs(stateId, page, limit)
        }
    }
    
    suspend fun getLGAById(lgaId: String): LGA {
        return withContext(Dispatchers.IO) {
            api.getLGAById(lgaId)
        }
    }
    
    // Wards API
    suspend fun getWards(lgaId: String, page: Int = 1, limit: Int = 20): PaginatedResponse<Ward> {
        return withContext(Dispatchers.IO) {
            api.getWards(lgaId, page, limit)
        }
    }
    
    suspend fun getWardById(wardId: String): Ward {
        return withContext(Dispatchers.IO) {
            api.getWardById(wardId)
        }
    }
    
    // Postal Codes API
    suspend fun getPostalCodes(wardId: String, page: Int = 1, limit: Int = 20): PaginatedResponse<PostalCode> {
        return withContext(Dispatchers.IO) {
            api.getPostalCodes(wardId, page, limit)
        }
    }
    
    // Search API
    suspend fun searchAll(query: String): SearchResult {
        return withContext(Dispatchers.IO) {
            api.search(query)
        }
    }
}