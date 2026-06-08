class Device {
  final String id;
  final String name;
  final String platform;
  final bool isOnline;
  final bool isLocked;
  final String? osVersion;
  final String? appVersion;
  final String? lastSeenAt;

  const Device({
    required this.id,
    required this.name,
    required this.platform,
    required this.isOnline,
    this.isLocked = false,
    this.osVersion,
    this.appVersion,
    this.lastSeenAt,
  });

  factory Device.fromJson(Map<String, dynamic> json) => Device(
        id: json['id'] as String,
        name: json['name'] as String,
        platform: json['platform'] as String,
        isOnline: (json['is_online'] as bool?) ?? false,
        isLocked: (json['is_locked'] as bool?) ?? false,
        osVersion: json['os_version'] as String?,
        appVersion: json['app_version'] as String?,
        lastSeenAt: json['last_seen_at'] as String?,
      );
}

class ClipboardEntry {
  final String id;
  final String contentType;
  final String contentHash;
  final int contentSize;
  final String createdAt;

  const ClipboardEntry({
    required this.id,
    required this.contentType,
    required this.contentHash,
    required this.contentSize,
    required this.createdAt,
  });

  factory ClipboardEntry.fromJson(Map<String, dynamic> json) => ClipboardEntry(
        id: json['id'] as String,
        contentType: json['content_type'] as String,
        contentHash: json['content_hash'] as String,
        contentSize: json['content_size'] as int,
        createdAt: json['created_at'] as String,
      );
}

class AuthTokens {
  final String accessToken;
  final String refreshToken;
  final int expiresIn;

  const AuthTokens({
    required this.accessToken,
    required this.refreshToken,
    required this.expiresIn,
  });

  factory AuthTokens.fromJson(Map<String, dynamic> json) => AuthTokens(
        accessToken: json['access_token'] as String,
        refreshToken: json['refresh_token'] as String,
        expiresIn: json['expires_in'] as int,
      );
}
