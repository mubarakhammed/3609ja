package com.nigeriageo.sdk.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.nigeriageo.sdk.models.NigerianState
import com.nigeriageo.sdk.NigeriaGeoSDK

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun StateDropdown(
    modifier: Modifier = Modifier,
    selectedState: NigerianState? = null,
    onStateSelected: (NigerianState) -> Unit = {},
    sdk: NigeriaGeoSDK = NigeriaGeoSDK.getInstance(),
    viewModel: StateDropdownViewModel = viewModel { StateDropdownViewModel(sdk) }
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    var expanded by remember { mutableStateOf(false) }
    
    LaunchedEffect(Unit) {
        viewModel.loadStates()
    }
    
    ExposedDropdownMenuBox(
        expanded = expanded,
        onExpandedChange = { expanded = it },
        modifier = modifier
    ) {
        OutlinedTextField(
            value = selectedState?.name ?: "Select State",
            onValueChange = {},
            readOnly = true,
            label = { Text("State") },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
            colors = ExposedDropdownMenuDefaults.outlinedTextFieldColors(),
            modifier = Modifier
                .menuAnchor()
                .fillMaxWidth()
        )
        
        ExposedDropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false }
        ) {
            when (uiState) {
                is StateDropdownUiState.Loading -> {
                    DropdownMenuItem(
                        text = { 
                            Row {
                                CircularProgressIndicator(
                                    modifier = Modifier.size(16.dp)
                                )
                                Spacer(modifier = Modifier.width(8.dp))
                                Text("Loading states...")
                            }
                        },
                        onClick = { }
                    )
                }
                is StateDropdownUiState.Success -> {
                    uiState.states.forEach { state ->
                        DropdownMenuItem(
                            text = { Text(state.name) },
                            onClick = {
                                onStateSelected(state)
                                expanded = false
                            }
                        )
                    }
                }
                is StateDropdownUiState.Error -> {
                    DropdownMenuItem(
                        text = { Text("Error: ${uiState.message}") },
                        onClick = { viewModel.loadStates() }
                    )
                }
            }
        }
    }
}