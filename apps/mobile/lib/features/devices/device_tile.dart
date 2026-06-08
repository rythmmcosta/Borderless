import 'package:flutter/material.dart';
import '../../core/models.dart';

class DeviceTile extends StatelessWidget {
  final Device device;
  const DeviceTile({super.key, required this.device});

  IconData get _icon {
    switch (device.platform.toLowerCase()) {
      case 'windows': return Icons.computer;
      case 'macos':   return Icons.laptop_mac;
      case 'linux':   return Icons.terminal;
      case 'android': return Icons.phone_android;
      case 'ios':     return Icons.phone_iphone;
      default:        return Icons.device_unknown;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: Icon(_icon, size: 32),
      title: Text(device.name),
      subtitle: Text(device.platform),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (device.isLocked)
            const Icon(Icons.lock, color: Colors.orange, size: 18),
          const SizedBox(width: 4),
          Container(
            width: 10, height: 10,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: device.isOnline ? Colors.green : Colors.grey,
            ),
          ),
        ],
      ),
    );
  }
}
