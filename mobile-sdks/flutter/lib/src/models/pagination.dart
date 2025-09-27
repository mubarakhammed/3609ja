class PaginatedResponse<T> {
  final List<T> data;
  final PaginationMeta pagination;

  const PaginatedResponse({
    required this.data,
    required this.pagination,
  });

  factory PaginatedResponse.fromJson(
    Map<String, dynamic> json,
    T Function(Object? json) fromJsonT,
  ) {
    final dataList = json['data'] as List<dynamic>? ?? [];
    return PaginatedResponse(
      data: dataList.map((item) => fromJsonT(item)).toList(),
      pagination:
          PaginationMeta.fromJson(json['pagination'] as Map<String, dynamic>),
    );
  }

  Map<String, dynamic> toJson(Object Function(T value) toJsonT) {
    return {
      'data': data.map((item) => toJsonT(item)).toList(),
      'pagination': pagination.toJson(),
    };
  }

  @override
  String toString() =>
      'PaginatedResponse(data: ${data.length} items, pagination: $pagination)';
}

class PaginationMeta {
  final int page;
  final int limit;
  final int total;
  final int totalPages;
  final bool hasNext;
  final bool hasPrev;

  const PaginationMeta({
    required this.page,
    required this.limit,
    required this.total,
    required this.totalPages,
    required this.hasNext,
    required this.hasPrev,
  });

  factory PaginationMeta.fromJson(Map<String, dynamic> json) {
    return PaginationMeta(
      page: json['page'] as int,
      limit: json['limit'] as int,
      total: json['total'] as int,
      totalPages: json['total_pages'] as int,
      hasNext: json['has_next'] as bool,
      hasPrev: json['has_prev'] as bool,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'page': page,
      'limit': limit,
      'total': total,
      'total_pages': totalPages,
      'has_next': hasNext,
      'has_prev': hasPrev,
    };
  }

  @override
  String toString() =>
      'PaginationMeta(page: $page, limit: $limit, total: $total, totalPages: $totalPages)';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is PaginationMeta &&
        other.page == page &&
        other.limit == limit &&
        other.total == total &&
        other.totalPages == totalPages &&
        other.hasNext == hasNext &&
        other.hasPrev == hasPrev;
  }

  @override
  int get hashCode =>
      Object.hash(page, limit, total, totalPages, hasNext, hasPrev);
}
