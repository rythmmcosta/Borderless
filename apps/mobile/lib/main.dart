import 'package:flutter/material.dart';
import 'core/api_client.dart';
import 'core/theme.dart';
import 'features/devices/devices_screen.dart';
import 'features/clipboard/clipboard_screen.dart';
import 'features/settings/settings_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await ApiClient().init();
  runApp(const BorderlessApp());
}

class BorderlessApp extends StatelessWidget {
  const BorderlessApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Borderless',
      theme: AppTheme.dark(),
      debugShowCheckedModeBanner: false,
      home: const MainShell(),
    );
  }
}

class MainShell extends StatefulWidget {
  const MainShell({super.key});

  @override
  State<MainShell> createState() => _MainShellState();
}

class _MainShellState extends State<MainShell> {
  int _tab = 0;

  static const _destinations = [
    NavigationDestination(icon: Icon(Icons.devices), label: 'Devices'),
    NavigationDestination(icon: Icon(Icons.content_paste), label: 'Clipboard'),
    NavigationDestination(icon: Icon(Icons.settings), label: 'Settings'),
  ];

  static const _screens = [
    DevicesScreen(),
    ClipboardScreen(),
    SettingsScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _screens[_tab],
      bottomNavigationBar: NavigationBar(
        selectedIndex: _tab,
        onDestinationSelected: (i) => setState(() => _tab = i),
        destinations: _destinations,
      ),
    );
  }
}
