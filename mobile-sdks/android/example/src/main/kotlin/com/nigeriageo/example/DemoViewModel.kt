package com.nigeriageo.example

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.nigeriageo.sdk.models.NigerianState
import com.nigeriageo.sdk.NigeriaGeoSDK
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class DemoUiState(
    val apiOutput: String = "Tap a button to test the API...",
    val selectedState: NigerianState? = null,
    val isLoading: Boolean = false
)

class DemoViewModel : ViewModel() {
    
    private val sdk = NigeriaGeoSDK.getInstance(enableLogging = true)
    
    private val _uiState = MutableStateFlow(DemoUiState())
    val uiState: StateFlow<DemoUiState> = _uiState.asStateFlow()
    
    fun testStatesAPI() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(
                isLoading = true,
                apiOutput = "Loading states..."
            )
            
            try {
                val response = sdk.getStates(limit = 5)
                val output = buildString {
                    appendLine("✅ States API Success!")
                    appendLine("Found ${response.data.size} states (page ${response.pagination.page}):")
                    appendLine()
                    response.data.forEach { state ->
                        appendLine("• ${state.name} (${state.code})")
                    }
                    appendLine()
                    appendLine("Total states: ${response.pagination.total}")
                }
                
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    apiOutput = output
                )
            } catch (e: Exception) {
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    apiOutput = "❌ Error: ${e.message}"
                )
            }
        }
    }
    
    fun testSearchAPI() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(
                isLoading = true,
                apiOutput = "Searching for 'Lagos'..."
            )
            
            try {
                val result = sdk.searchAll("Lagos")
                val output = buildString {
                    appendLine("✅ Search API Success!")
                    appendLine("Query: \"Lagos\"")
                    appendLine()
                    appendLine("States found: ${result.states.size}")
                    appendLine("LGAs found: ${result.lgas.size}")
                    appendLine("Wards found: ${result.wards.size}")
                    appendLine("Postal codes found: ${result.postalCodes.size}")
                    appendLine()
                    appendLine("Sample results:")
                    result.states.take(2).forEach { state ->
                        appendLine("• State: ${state.name}")
                    }
                    result.lgas.take(2).forEach { lga ->
                        appendLine("• LGA: ${lga.name}")
                    }
                }
                
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    apiOutput = output
                )
            } catch (e: Exception) {
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    apiOutput = "❌ Search Error: ${e.message}"
                )
            }
        }
    }
    
    fun onStateSelected(state: NigerianState) {
        _uiState.value = _uiState.value.copy(selectedState = state)
    }
}