import 'package:flutter/material.dart';

class KvmScreen extends StatefulWidget {
  const KvmScreen({super.key});

  @override
  State<KvmScreen> createState() => _KvmScreenState();
}

class _KvmScreenState extends State<KvmScreen> {
  bool _connected = false;
  Offset _cursor = Offset.zero;

  void _onPan(DragUpdateDetails d) {
    setState(() => _cursor = d.globalPosition);
    // TODO: send delta over WebSocket to target device
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('KVM Control'),
        actions: [
          IconButton(
            tooltip: _connected ? 'Disconnect' : 'Connect',
            icon: Icon(_connected ? Icons.link_off : Icons.link),
            onPressed: () => setState(() => _connected = !_connected),
          ),
        ],
      ),
      body: Column(
        children: [
          _StatusBanner(connected: _connected),
          Expanded(
            child: _connected ? _Touchpad(onPan: _onPan, cursor: _cursor) : _EmptyState(),
          ),
        ],
      ),
    );
  }
}

class _StatusBanner extends StatelessWidget {
  final bool connected;
  const _StatusBanner({required this.connected});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
      color: connected
          ? const Color(0xFF22C55E).withOpacity(0.1)
          : Colors.grey.withOpacity(0.1),
      child: Row(
        children: [
          Icon(
            connected ? Icons.check_circle : Icons.radio_button_unchecked,
            color: connected ? const Color(0xFF22C55E) : Colors.grey,
            size: 16,
          ),
          const SizedBox(width: 8),
          Text(
            connected ? 'Session active' : 'Not connected — tap link icon to start',
            style: TextStyle(
              color: connected ? const Color(0xFF22C55E) : Colors.grey,
              fontSize: 13,
            ),
          ),
        ],
      ),
    );
  }
}

class _Touchpad extends StatelessWidget {
  final void Function(DragUpdateDetails) onPan;
  final Offset cursor;
  const _Touchpad({required this.onPan, required this.cursor});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onPanUpdate: onPan,
      child: Container(
        color: Colors.black87,
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(Icons.touch_app, size: 64, color: Colors.white24),
              const SizedBox(height: 16),
              const Text('Drag to move cursor', style: TextStyle(color: Colors.white38)),
              const SizedBox(height: 8),
              Text(
                'x: ${cursor.dx.toInt()}  y: ${cursor.dy.toInt()}',
                style: const TextStyle(color: Colors.white24, fontSize: 12),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _EmptyState extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.mouse, size: 64, color: Colors.grey),
          const SizedBox(height: 16),
          const Text('Select a device and tap link to start KVM',
              style: TextStyle(color: Colors.grey)),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: () => Navigator.pop(context),
            icon: const Icon(Icons.devices),
            label: const Text('Back to Devices'),
          ),
        ],
      ),
    );
  }
}
