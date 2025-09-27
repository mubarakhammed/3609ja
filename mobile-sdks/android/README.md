# 🤖 Nigeria Geo Android SDK

Native Kotlin/Java SDK for integrating Nigerian geographic data into Android applications.

## 🚀 Features

- ✅ Native Kotlin implementation with Java interoperability
- 🗺️ Complete geographic hierarchy (States → LGAs → Wards → Postal Codes)
- 📍 Location services integration with FusedLocationProviderClient
- 💾 Intelligent caching with Room database
- 🔍 Smart search with RxJava and autocomplete
- ✅ Real-time address validation
- 📱 Jetpack Compose UI components
- 🧵 Coroutines and Flow support
- 🛡️ Comprehensive error handling
- 🎨 Material Design 3 components

## 📋 Requirements

- Android API level 21 (Android 5.0) or higher
- Kotlin 1.7.0+
- Java 8+ (for Java projects)

## 🛠️ Installation

### Gradle (Kotlin DSL)

Add to your module's `build.gradle.kts`:

```kotlin
dependencies {
    implementation("com.nigeriago:sdk:1.0.0")
    
    // Optional: UI components (Jetpack Compose)
    implementation("com.nigeriago:sdk-compose:1.0.0")
    
    // Optional: UI components (View system)
    implementation("com.nigeriago:sdk-views:1.0.0")
}
```

### Gradle (Groovy)

Add to your module's `build.gradle`:

```gradle
dependencies {
    implementation 'com.nigeriago:sdk:1.0.0'
    
    // Optional: UI components (Jetpack Compose)
    implementation 'com.nigeriago:sdk-compose:1.0.0'
    
    // Optional: UI components (View system)  
    implementation 'com.nigeriago:sdk-views:1.0.0'
}
```

## 🎯 Quick Start

### 1. Initialize the SDK

#### In Application class:

```kotlin
import com.nigeriago.sdk.NigeriaGeoSDK
import com.nigeriago.sdk.NigeriaGeoConfig

class MyApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        
        val config = NigeriaGeoConfig.Builder(this)
            .baseUrl("https://your-api-base-url.com")
            .apiKey("your-api-key") // Optional
            .enableCaching(true)
            .cacheExpiration(24, TimeUnit.HOURS)
            .requestTimeout(30, TimeUnit.SECONDS)
            .enableLogging(BuildConfig.DEBUG)
            .build()
            
        NigeriaGeoSDK.initialize(config)
    }
}
```

### 2. Basic Usage

#### Get the client instance:

```kotlin
import com.nigeriago.sdk.NigeriaGeoClient

class MainActivity : AppCompatActivity() {
    private val geoClient = NigeriaGeoSDK.getClient()
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Get all states
        loadStates()
    }
    
    private fun loadStates() {
        lifecycleScope.launch {
            try {
                val states = geoClient.getStates()
                // Update UI with states
                updateStatesUI(states.data)
            } catch (e: NigeriaGeoException) {
                // Handle error
                showError(e.message)
            }
        }
    }
}
```

### 3. Using Callbacks (Java/Kotlin)

```kotlin
// Kotlin with callbacks
geoClient.getStates(object : NigeriaGeoCallback<PaginatedResponse<State>> {
    override fun onSuccess(result: PaginatedResponse<State>) {
        // Handle success
        val states = result.data
        updateUI(states)
    }
    
    override fun onError(error: NigeriaGeoException) {
        // Handle error
        showError(error.message)
    }
})

// Java
geoClient.getStates(new NigeriaGeoCallback<PaginatedResponse<State>>() {
    @Override
    public void onSuccess(PaginatedResponse<State> result) {
        // Handle success
        List<State> states = result.getData();
        updateUI(states);
    }
    
    @Override
    public void onError(NigeriaGeoException error) {
        // Handle error
        showError(error.getMessage());
    }
});
```

### 4. Using Flow (Kotlin)

```kotlin
class StateRepository {
    private val geoClient = NigeriaGeoSDK.getClient()
    
    fun getStatesFlow(): Flow<List<State>> = flow {
        try {
            val response = geoClient.getStates()
            emit(response.data)
        } catch (e: Exception) {
            throw e
        }
    }.flowOn(Dispatchers.IO)
    
    fun searchStatesFlow(query: String): Flow<List<State>> = flow {
        try {
            val searchResult = geoClient.searchAll(query)
            emit(searchResult.states)
        } catch (e: Exception) {
            throw e
        }
    }.flowOn(Dispatchers.IO)
}
```

## 🎨 Jetpack Compose UI Components

### StatePicker Composable

```kotlin
import com.nigeriago.sdk.compose.StatePicker
import com.nigeriago.sdk.compose.rememberStatePickerState

@Composable
fun AddressForm() {
    var selectedState by remember { mutableStateOf<State?>(null) }
    val statePickerState = rememberStatePickerState()
    
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp)
    ) {
        StatePicker(
            state = statePickerState,
            selectedState = selectedState,
            onStateSelected = { selectedState = it },
            modifier = Modifier.fillMaxWidth()
        )
        
        // Show LGA picker when state is selected
        selectedState?.let { state ->
            Spacer(modifier = Modifier.height(16.dp))
            
            LGAPicker(
                selectedState = state,
                onLGASelected = { lga ->
                    // Handle LGA selection
                }
            )
        }
    }
}
```

### Address Search with Autocomplete

```kotlin
@Composable
fun LocationSearchScreen() {
    var searchQuery by remember { mutableStateOf("") }
    var searchResults by remember { mutableStateOf<List<SearchResultItem>>(emptyList()) }
    val geoClient = NigeriaGeoSDK.getClient()
    
    LaunchedEffect(searchQuery) {
        if (searchQuery.length >= 2) {
            delay(300) // Debounce
            try {
                val result = geoClient.searchAll(searchQuery)
                searchResults = result.toSearchResultItems()
            } catch (e: Exception) {
                // Handle error
            }
        } else {
            searchResults = emptyList()
        }
    }
    
    Column {
        OutlinedTextField(
            value = searchQuery,
            onValueChange = { searchQuery = it },
            label = { Text("Search locations...") },
            leadingIcon = { 
                Icon(Icons.Default.Search, contentDescription = null) 
            },
            modifier = Modifier.fillMaxWidth()
        )
        
        LazyColumn {
            items(searchResults) { item ->
                SearchResultItem(
                    item = item,
                    onClick = { 
                        // Handle item selection
                    }
                )
            }
        }
    }
}
```

### Address Validation Form

```kotlin
@Composable
fun AddressValidationForm() {
    var addressState by remember { mutableStateOf("") }
    var addressLGA by remember { mutableStateOf("") }
    var addressWard by remember { mutableStateOf("") }
    var postalCode by remember { mutableStateOf("") }
    var validationResult by remember { mutableStateOf<AddressValidationResponse?>(null) }
    
    val geoClient = NigeriaGeoSDK.getClient()
    
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        OutlinedTextField(
            value = addressState,
            onValueChange = { addressState = it },
            label = { Text("State") },
            modifier = Modifier.fillMaxWidth()
        )
        
        OutlinedTextField(
            value = addressLGA,
            onValueChange = { addressLGA = it },
            label = { Text("LGA") },
            modifier = Modifier.fillMaxWidth()
        )
        
        OutlinedTextField(
            value = addressWard,
            onValueChange = { addressWard = it },
            label = { Text("Ward (Optional)") },
            modifier = Modifier.fillMaxWidth()
        )
        
        OutlinedTextField(
            value = postalCode,
            onValueChange = { postalCode = it },
            label = { Text("Postal Code (Optional)") },
            modifier = Modifier.fillMaxWidth()
        )
        
        Button(
            onClick = {
                // Validate address
                lifecycleScope.launch {
                    try {
                        val request = AddressValidationRequest(
                            state = addressState,
                            lga = addressLGA,
                            ward = addressWard.takeIf { it.isNotBlank() },
                            postalCode = postalCode.takeIf { it.isNotBlank() }
                        )
                        validationResult = geoClient.validateAddress(request)
                    } catch (e: Exception) {
                        // Handle error
                    }
                }
            },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("Validate Address")
        }
        
        // Show validation result
        validationResult?.let { result ->
            AddressValidationResult(result)
        }
    }
}

@Composable
fun AddressValidationResult(result: AddressValidationResponse) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = if (result.isValid) {
                MaterialTheme.colorScheme.primaryContainer
            } else {
                MaterialTheme.colorScheme.errorContainer
            }
        )
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically
            ) {
                Icon(
                    imageVector = if (result.isValid) Icons.Default.CheckCircle else Icons.Default.Error,
                    contentDescription = null,
                    tint = if (result.isValid) {
                        MaterialTheme.colorScheme.onPrimaryContainer
                    } else {
                        MaterialTheme.colorScheme.onErrorContainer
                    }
                )
                Spacer(modifier = Modifier.width(8.dp))
                Text(
                    text = if (result.isValid) "Valid Address" else "Invalid Address",
                    style = MaterialTheme.typography.titleMedium
                )
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Text(
                text = "Confidence: ${(result.confidence * 100).toInt()}%",
                style = MaterialTheme.typography.bodyMedium
            )
            
            // Show suggestions if any
            if (result.suggestions.isNotEmpty()) {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Suggestions:",
                    style = MaterialTheme.typography.titleSmall
                )
                
                result.suggestions.forEach { suggestion ->
                    Text(
                        text = "${suggestion.field}: ${suggestion.suggestedValue}",
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }
    }
}
```

## 📱 View System Components

### StateSpinner (XML Layouts)

```xml
<!-- In your layout file -->
<com.nigeriago.sdk.views.StateSpinner
    android:id="@+id/stateSpinner"
    android:layout_width="match_parent"
    android:layout_height="wrap_content"
    android:layout_margin="16dp"
    app:hint="Select State"
    app:enableSearch="true" />
```

```kotlin
// In your Activity/Fragment
class AddressActivity : AppCompatActivity() {
    private lateinit var stateSpinner: StateSpinner
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_address)
        
        stateSpinner = findViewById(R.id.stateSpinner)
        stateSpinner.setOnStateSelectedListener { state ->
            // Handle state selection
            loadLGAsForState(state.id)
        }
    }
}
```

### LocationSearchView

```xml
<com.nigeriago.sdk.views.LocationSearchView
    android:id="@+id/locationSearchView"
    android:layout_width="match_parent"
    android:layout_height="wrap_content"
    android:layout_margin="16dp"
    app:hint="Search for location..."
    app:maxSuggestions="10"
    app:debounceDelay="300" />
```

```kotlin
locationSearchView.setOnLocationSelectedListener { location ->
    when (location.type) {
        SearchResultType.STATE -> {
            val state = location.data as State
            // Handle state selection
        }
        SearchResultType.LGA -> {
            val lga = location.data as LGA
            // Handle LGA selection
        }
        // ... other types
    }
}
```

## 📍 Location Services Integration

### Get Current Location Context

```kotlin
import com.nigeriago.sdk.location.LocationManager
import com.nigeriago.sdk.location.LocationCallback

class LocationActivity : AppCompatActivity() {
    private lateinit var locationManager: LocationManager
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        locationManager = LocationManager(this)
        
        // Request location permission first
        requestLocationPermission()
    }
    
    private fun getCurrentLocationContext() {
        locationManager.getCurrentLocationContext(object : LocationCallback {
            override fun onLocationDetected(
                latitude: Double, 
                longitude: Double, 
                nearbyPostalCodes: List<PostalCode>
            ) {
                // Handle location detection
                showNearbyLocations(nearbyPostalCodes)
            }
            
            override fun onLocationError(error: LocationException) {
                // Handle error
                showLocationError(error.message)
            }
        })
    }
    
    private fun requestLocationPermission() {
        if (ContextCompat.checkSelfPermission(
                this, 
                Manifest.permission.ACCESS_FINE_LOCATION
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            ActivityCompat.requestPermissions(
                this,
                arrayOf(Manifest.permission.ACCESS_FINE_LOCATION),
                LOCATION_PERMISSION_REQUEST_CODE
            )
        } else {
            getCurrentLocationContext()
        }
    }
}
```

## 💾 Caching and Offline Support

### Cache Configuration

```kotlin
val config = NigeriaGeoConfig.Builder(context)
    .enableCaching(true)
    .cacheExpiration(24, TimeUnit.HOURS)
    .maxCacheSize(50, SizeUnit.MEGABYTES)
    .offlineModeEnabled(true)
    .build()
```

### Manual Cache Management

```kotlin
val cacheManager = NigeriaGeoSDK.getCacheManager()

// Clear all cache
cacheManager.clearAll()

// Clear specific cache
cacheManager.clearStatesCache()
cacheManager.clearSearchCache()

// Get cache statistics
val stats = cacheManager.getStatistics()
println("Cache size: ${stats.sizeInMegabytes} MB")
println("Hit rate: ${stats.hitRate}%")
```

## 🛡️ Error Handling

### Exception Types

```kotlin
sealed class NigeriaGeoException : Exception() {
    data class NetworkException(override val message: String) : NigeriaGeoException()
    data class InvalidResponseException(override val message: String) : NigeriaGeoException()
    data class UnauthorizedException(override val message: String) : NigeriaGeoException()
    data class NotFoundException(override val message: String) : NigeriaGeoException()
    data class RateLimitException(override val message: String) : NigeriaGeoException()
    data class CacheException(override val message: String) : NigeriaGeoException()
    data class LocationException(override val message: String) : NigeriaGeoException()
}
```

### Error Recovery

```kotlin
lifecycleScope.launch {
    try {
        val states = geoClient.getStates()
        updateUI(states.data)
    } catch (e: NigeriaGeoException) {
        when (e) {
            is NigeriaGeoException.NetworkException -> {
                // Try to load from cache
                loadStatesFromCache()
            }
            is NigeriaGeoException.RateLimitException -> {
                // Show rate limit message
                showRateLimitError()
            }
            is NigeriaGeoException.UnauthorizedException -> {
                // Show authentication error
                showAuthError()
            }
            else -> {
                // Show generic error
                showGenericError(e.message)
            }
        }
    }
}
```

## 🧪 Testing

### Unit Tests

```kotlin
@RunWith(AndroidJUnit4::class)
class NigeriaGeoClientTest {
    
    private lateinit var geoClient: NigeriaGeoClient
    private lateinit var mockServer: MockWebServer
    
    @Before
    fun setup() {
        mockServer = MockWebServer()
        mockServer.start()
        
        val config = NigeriaGeoConfig.Builder(ApplicationProvider.getApplicationContext())
            .baseUrl(mockServer.url("/").toString())
            .enableCaching(false)
            .build()
            
        NigeriaGeoSDK.initialize(config)
        geoClient = NigeriaGeoSDK.getClient()
    }
    
    @Test
    fun testGetStates() = runTest {
        // Mock response
        val mockResponse = MockResponse()
            .setBody(loadJsonFromAssets("states_response.json"))
            .setResponseCode(200)
        mockServer.enqueue(mockResponse)
        
        // Test
        val response = geoClient.getStates()
        
        // Verify
        assertThat(response.data).isNotEmpty()
        assertThat(response.pagination.total).isEqualTo(37)
    }
    
    @Test
    fun testSearchStates() = runTest {
        val mockResponse = MockResponse()
            .setBody(loadJsonFromAssets("search_response.json"))
            .setResponseCode(200)
        mockServer.enqueue(mockResponse)
        
        val result = geoClient.searchAll("lagos")
        
        assertThat(result.states).isNotEmpty()
        assertThat(result.states.first().name).contains("Lagos")
    }
    
    @After
    fun teardown() {
        mockServer.shutdown()
    }
}
```

### Integration Tests

```kotlin
@RunWith(AndroidJUnit4::class)
@LargeTest
class NigeriaGeoIntegrationTest {
    
    @Test
    fun testRealAPIIntegration() = runTest {
        val config = NigeriaGeoConfig.Builder(ApplicationProvider.getApplicationContext())
            .baseUrl("https://api.nigeria-geo.com")
            .build()
            
        NigeriaGeoSDK.initialize(config)
        val geoClient = NigeriaGeoSDK.getClient()
        
        // Test real API call
        val states = geoClient.getStates()
        assertThat(states.data).isNotEmpty()
    }
}
```

## 📊 Performance Optimization

### Best Practices

1. **Use Pagination**: Load data in chunks
2. **Enable Caching**: Reduce network requests
3. **Use Flow/LiveData**: For reactive UI updates
4. **Background Processing**: Use coroutines for API calls
5. **Image Loading**: Optimize image sizes and caching

### Performance Monitoring

```kotlin
// Enable performance monitoring
val config = NigeriaGeoConfig.Builder(context)
    .enablePerformanceMonitoring(true)
    .performanceListener { metrics ->
        // Log performance metrics
        Log.d("Performance", "Average response time: ${metrics.averageResponseTime}ms")
        Log.d("Performance", "Cache hit rate: ${metrics.cacheHitRate}%")
    }
    .build()
```

## 📚 Example Projects

Check out our example applications:

- **BasicIntegration** - Simple usage examples
- **ComposeApp** - Modern Jetpack Compose application
- **ViewSystemApp** - Traditional View system implementation
- **DeliveryApp** - Complete delivery application with location services
- **AddressBook** - Contact management with address validation

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## 📞 Support

- **📧 Email**: android-sdk@nigeria-geo.com
- **🐛 Issues**: [GitHub Issues](../../issues)
- **💬 Discussions**: [GitHub Discussions](../../discussions)
- **📖 Documentation**: [Full Documentation](https://docs.nigeria-geo.com/android)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.