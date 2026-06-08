import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'models.dart';

class ApiClient {
  static final ApiClient _instance = ApiClient._();
  factory ApiClient() => _instance;
  ApiClient._();

  String _baseUrl = 'http://localhost:8080/v1';
  String? _accessToken;

  Future<void> init() async {
    final prefs = await SharedPreferences.getInstance();
    _baseUrl = prefs.getString('server_url') ?? 'http://localhost:8080/v1';
    _accessToken = prefs.getString('access_token');
  }

  Future<void> setBaseUrl(String url) async {
    _baseUrl = url.trim();
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('server_url', _baseUrl);
  }

  String get baseUrl => _baseUrl;
  bool get isLoggedIn => _accessToken != null;

  Map<String, String> get _headers => {
        'Content-Type': 'application/json',
        if (_accessToken != null) 'Authorization': 'Bearer $_accessToken',
      };

  Future<AuthTokens> login(String email, String password) async {
    final res = await http.post(
      Uri.parse('$_baseUrl/auth/login'),
      headers: _headers,
      body: jsonEncode({'email': email, 'password': password}),
    );
    if (res.statusCode != 200) throw Exception('Login failed: ${res.body}');
    final tokens = AuthTokens.fromJson(jsonDecode(res.body) as Map<String, dynamic>);
    _accessToken = tokens.accessToken;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('access_token', tokens.accessToken);
    await prefs.setString('refresh_token', tokens.refreshToken);
    return tokens;
  }

  Future<void> logout() async {
    final prefs = await SharedPreferences.getInstance();
    final refresh = prefs.getString('refresh_token');
    if (refresh != null) {
      await http.post(
        Uri.parse('$_baseUrl/auth/logout'),
        headers: _headers,
        body: jsonEncode({'refresh_token': refresh}),
      );
    }
    _accessToken = null;
    await prefs.remove('access_token');
    await prefs.remove('refresh_token');
  }

  Future<List<Device>> listDevices() async {
    final res = await http.get(Uri.parse('$_baseUrl/devices'), headers: _headers);
    if (res.statusCode != 200) throw Exception('Failed to load devices: ${res.statusCode}');
    final data = jsonDecode(res.body) as List<dynamic>;
    return data.map((j) => Device.fromJson(j as Map<String, dynamic>)).toList();
  }

  Future<List<ClipboardEntry>> listClipboard({int limit = 50}) async {
    final res = await http.get(
      Uri.parse('$_baseUrl/clipboard?limit=$limit'),
      headers: _headers,
    );
    if (res.statusCode != 200) throw Exception('Failed to load clipboard: ${res.statusCode}');
    final data = jsonDecode(res.body) as List<dynamic>;
    return data.map((j) => ClipboardEntry.fromJson(j as Map<String, dynamic>)).toList();
  }
}
