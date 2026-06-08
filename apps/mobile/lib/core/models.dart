class Device {
  final String id;
  final String name;
  final String platform;
  final bool isOnline;
  final bool isLocked;

  const Device({
    required this.id,
    required this.name,
    required this.platform,
    required this.isOnline,
    required this.isLocked,
  });

  factory Device.fromJson(Map<String, dynamic> json) => Device(
        id: json['id'] as String,
        name: json['name'] as String,
        platform: json['platform'] as String,
        isOnline: json['is_online'] as bool? ?? false,
        isLocked: json['is_locked'] as bool? ?? false,
      );
}

class ClipboardEntry {
  final String id;
  final String contentType;
  final int contentSize;
  final String createdAt;

  const ClipboardEntry({
    required this.id,
    required this.contentType,
    required this.contentSize,
    required this.createdAt,
  });

  factory ClipboardEntry.fromJson(Map<String, dynamic> json) => ClipboardEntry(
        id: json['id'] as String,
        contentType: json['content_type'] as String? ?? 'text',
        contentSize: json['content_size'] as int? ?? 0,
        createdAt: json['created_at'] as String? ?? '',
      );
}
