import 'dart:async';

/// Utility class for debouncing function calls
class Debouncer {
  final Duration delay;
  Timer? _timer;

  Debouncer({required this.delay});

  /// Debounce a function call
  void call(void Function() callback) {
    _timer?.cancel();
    _timer = Timer(delay, callback);
  }

  /// Cancel any pending debounced calls
  void cancel() {
    _timer?.cancel();
    _timer = null;
  }

  /// Check if there's a pending call
  bool get isActive => _timer?.isActive == true;

  /// Dispose the debouncer
  void dispose() {
    cancel();
  }
}

/// Utility for debouncing async operations with result caching
class AsyncDebouncer<T> {
  final Duration delay;
  Timer? _timer;
  Completer<T>? _completer;

  AsyncDebouncer({required this.delay});

  /// Debounce an async function call
  Future<T> call(Future<T> Function() callback) {
    // Cancel any existing timer
    _timer?.cancel();

    // If there's already a completer, complete it with an error
    if (_completer != null && !_completer!.isCompleted) {
      _completer!.completeError(
        StateError('Debounced call was cancelled by newer call'),
      );
    }

    // Create new completer
    _completer = Completer<T>();

    // Set up new timer
    _timer = Timer(delay, () async {
      try {
        final result = await callback();
        if (!_completer!.isCompleted) {
          _completer!.complete(result);
        }
      } catch (error) {
        if (!_completer!.isCompleted) {
          _completer!.completeError(error);
        }
      }
    });

    return _completer!.future;
  }

  /// Cancel any pending debounced calls
  void cancel() {
    _timer?.cancel();
    _timer = null;

    if (_completer != null && !_completer!.isCompleted) {
      _completer!.completeError(
        StateError('Debounced call was cancelled'),
      );
    }
    _completer = null;
  }

  /// Check if there's a pending call
  bool get isActive => _timer?.isActive == true;

  /// Dispose the debouncer
  void dispose() {
    cancel();
  }
}

/// Search debouncer specifically designed for search queries
class SearchDebouncer {
  final Duration delay;
  Timer? _timer;
  Completer<dynamic>? _completer;

  SearchDebouncer({Duration? delay})
      : delay = delay ?? const Duration(milliseconds: 300);

  /// Debounce search with query validation
  Future<T> search<T>(String query, Future<T> Function(String) searchFunction) {
    final trimmedQuery = query.trim();

    if (trimmedQuery.isEmpty) {
      return Future.error(ArgumentError('Search query cannot be empty'));
    }

    if (trimmedQuery.length < 2) {
      return Future.error(
          ArgumentError('Search query must be at least 2 characters'));
    }

    // Cancel any existing timer
    _timer?.cancel();

    // If there's already a completer, complete it with an error
    if (_completer != null && !_completer!.isCompleted) {
      _completer!.completeError(
        StateError('Search was cancelled by newer search'),
      );
    }

    // Create new completer
    _completer = Completer<T>();

    // Set up new timer
    _timer = Timer(delay, () async {
      try {
        final result = await searchFunction(trimmedQuery);
        if (!_completer!.isCompleted) {
          _completer!.complete(result);
        }
      } catch (error) {
        if (!_completer!.isCompleted) {
          _completer!.completeError(error);
        }
      }
    });

    return _completer!.future as Future<T>;
  }

  /// Cancel any pending search
  void cancel() {
    _timer?.cancel();
    _timer = null;

    if (_completer != null && !_completer!.isCompleted) {
      _completer!.completeError(
        StateError('Search was cancelled'),
      );
    }
    _completer = null;
  }

  /// Check if there's a pending search
  bool get isActive => _timer?.isActive == true;

  /// Dispose the debouncer
  void dispose() {
    cancel();
  }
}
