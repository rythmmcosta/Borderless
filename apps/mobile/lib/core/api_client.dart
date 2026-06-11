import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'models.dart';

class ApiClient extends ChangeNotifier {
  final SharedPreferences prefs;
  String? _token;

  ApiClient({required this.prefs}) {
    _token = prefs.getString('auth_token');
  }

  String get baseUrl => prefs.getString('server_url') ?? 'http://localhost:8080';
  bool get isAuthenticated => _token != null;

  Map<String, String> get _headers => {
        'Content-Type': 'application/json',
        if (_token != null) 'Authorization': 'Bearer $_token',
      };

  Future<void> login(String email, String password) async {
    final res = await http.post(
      Uri.parse('$baseUrl/v1/auth/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'email': email, 'password': password}),
    );
    if (res.statusCode == 200) {
      final data = jsonDecode(res.body) as Map<String, dynamic>;
      _token = data['access_token'] as String?;
      if (_token == null) throw Exception('Login failed: no access_token in response');
      await prefs.setString('auth_token', _token!);
      notifyListeners();
    } else {
      throw Exception('Login failed: ${res.body}');
    }
  }

  Future<void> logout() async {
    await http.post(Uri.parse('$baseUrl/v1/auth/logout'), headers: _headers);
    _token = null;
    await prefs.remove('auth_token');
    notifyListeners();
  }

  Future<List<Device>> getDevices() async {
    final res = await http.get(Uri.parse('$baseUrl/v1/devices'), headers: _headers);
    if (res.statusCode == 200) {
      final list = jsonDecode(res.body) as List<dynamic>;
      return list.map((e) => Device.fromJson(e as Map<String, dynamic>)).toList();
    }
    throw Exception('Failed to fetch devices: ${res.statusCode}');
  }

  Future<List<ClipboardEntry>> getClipboard() async {
    final res = await http.get(Uri.parse('$baseUrl/v1/clipboard'), headers: _headers);
    if (res.statusCode == 200) {
      final list = jsonDecode(res.body) as List<dynamic>;
      return list.map((e) => ClipboardEntry.fromJson(e as Map<String, dynamic>)).toList();
    }
    throw Exception('Failed to fetch clipboard: ${res.statusCode}');
  }

  Future<void> updateServerUrl(String url) async {
    await prefs.setString('server_url', url);
    notifyListeners();
  }
}
