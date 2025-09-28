package com.nigeriageo.sdk.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.nigeriageo.sdk.models.NigerianState
import com.nigeriageo.sdk.NigeriaGeoSDK
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

sealed class StateDropdownUiState {
    object Loading : StateDropdownUiState()
    data class Success(val states: List<NigerianState>) : StateDropdownUiState()
    data class Error(val message: String) : StateDropdownUiState()
}

class StateDropdownViewModel(
    private val sdk: NigeriaGeoSDK
) : ViewModel() {
    
    private val _uiState = MutableStateFlow<StateDropdownUiState>(StateDropdownUiState.Loading)
    val uiState: StateFlow<StateDropdownUiState> = _uiState.asStateFlow()
    
    fun loadStates() {
        viewModelScope.launch {
            _uiState.value = StateDropdownUiState.Loading
            try {
                val response = sdk.getStates(limit = 50) // Get all states
                _uiState.value = StateDropdownUiState.Success(response.data)
            } catch (e: Exception) {
                _uiState.value = StateDropdownUiState.Error(e.message ?: "Unknown error occurred")
            }
        }
    }
}