import 'package:flutter/material.dart';
import '../../core/models.dart';

class DeviceTile extends StatelessWidget {
  final Device device;
  const DeviceTile({super.key, required this.device});

  IconData _icon(String platform) {
    switch (platform.toLowerCase()) {
      case 'windows':  return Icons.desktop_windows;
      case 'macos':    return Icons.desktop_mac;
      case 'linux':    return Icons.computer;
      case 'android':  return Icons.phone_android;
      case 'ios':      return Icons.phone_iphone;
      default:         return Icons.devices;
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final onlineColor = device.isOnline ? const Color(0xFF22C55E) : Colors.grey;
    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: onlineColor.withOpacity(0.15),
          child: Icon(_icon(device.platform), color: onlineColor, size: 22),
        ),
        title: Text(
          device.name,
          style: const TextStyle(fontWeight: FontWeight.w600),
        ),
        subtitle: Text(
          '${device.platform}${device.osVersion != null ? ' ${device.osVersion}' : ''}  •  '
          '${device.isOnline ? "Online" : "Offline"}',
          style: TextStyle(
            color: theme.colorScheme.onSurface.withOpacity(0.55),
            fontSize: 12,
          ),
        ),
        trailing: device.isLocked
            ? const Icon(Icons.lock, color: Colors.orange, size: 20)
            : Icon(
                Icons.circle,
                color: onlineColor,
                size: 10,
              ),
      ),
    );
  }
}
