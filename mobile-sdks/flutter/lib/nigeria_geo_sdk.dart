// Main SDK export file
library nigeria_geo_sdk;

import 'src/client/nigeria_geo_client.dart';
import 'src/client/nigeria_geo_config.dart';
import 'src/services/cache_service.dart';
import 'src/config/sdk_config_loader.dart';
import 'package:http/http.dart' as http;

// Core client
export 'src/client/nigeria_geo_client.dart';
export 'src/client/nigeria_geo_config.dart';
export 'src/client/cache_manager.dart';

// Models
export 'src/models/state.dart';
export 'src/models/lga.dart';
export 'src/models/ward.dart';
export 'src/models/postal_code.dart';
export 'src/models/search_result.dart';
export 'src/models/address_validation.dart';
export 'src/models/pagination.dart';
export 'src/models/location_context.dart';

// Exceptions
export 'src/exceptions/nigeria_geo_exception.dart';

// Services
export 'src/services/location_service.dart';
export 'src/services/cache_service.dart';

// Configuration
export 'src/config/sdk_config_loader.dart';

// Widgets
export 'src/widgets/widgets.dart';

// Utils
export 'src/utils/debouncer.dart';
export 'src/utils/logger.dart';

// Main SDK initialization
export 'src/nigeria_geo_sdk.dart';
