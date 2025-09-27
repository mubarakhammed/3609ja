import 'package:logger/logger.dart';

/// Singleton logger for the Nigeria Geo SDK
class NigeriaGeoLogger {
  static Logger? _instance;
  static bool _isInitialized = false;

  /// Get the logger instance
  static Logger get instance {
    if (_instance == null) {
      throw StateError(
          'Logger not initialized. Call NigeriaGeoLogger.initialize() first.');
    }
    return _instance!;
  }

  /// Initialize the logger
  static void initialize(bool enableLogging) {
    if (_isInitialized) return;

    if (enableLogging) {
      _instance = Logger(
        filter: ProductionFilter(),
        printer: PrettyPrinter(
          methodCount: 2,
          errorMethodCount: 8,
          lineLength: 120,
          colors: true,
          printEmojis: true,
          printTime: true,
        ),
        output: ConsoleOutput(),
      );
    } else {
      _instance = Logger(
        filter: ProductionFilter(),
        printer: SimplePrinter(),
        output: ConsoleOutput(),
        level: Level.nothing,
      );
    }

    _isInitialized = true;
  }

  /// Check if logger is initialized
  static bool get isInitialized => _isInitialized;

  /// Dispose logger resources
  static void dispose() {
    _instance?.close();
    _instance = null;
    _isInitialized = false;
  }
}

/// Custom filter for production logging
class ProductionFilter extends LogFilter {
  @override
  bool shouldLog(LogEvent event) {
    // In production, only log warnings and errors
    return event.level.index >= Level.warning.index;
  }
}

/// Custom output that adds SDK prefix
class ConsoleOutput extends LogOutput {
  @override
  void output(OutputEvent event) {
    for (var line in event.lines) {
      print('[NigeriaGeoSDK] $line');
    }
  }
}
