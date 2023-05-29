import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart';

class HttpApi {
  final String baseUrl;
  final Client client = Client();

  HttpApi({this.baseUrl = 'http://localhost:3000/api'});

  Future<dynamic> get(String path) async {
    var res = await client.get(Uri.parse("$baseUrl/$path"));
    if (res.statusCode == 200) {
      return json.decode(utf8.decode(res.bodyBytes));
    } else {
      _throwHttpError(res);
    }
  }

  Future<dynamic> post(String path, Object body) async {
    var res = await client.post(Uri.parse("$baseUrl/$path"),
        body: json.encode(body), headers: {"Content-Type": "application/json"});
    if (res.statusCode == 201) {
      return json.decode(utf8.decode(res.bodyBytes));
    } else {
      _throwHttpError(res);
    }
  }

  Future<dynamic> put(String path, Object body) async {
    var res = await client.put(Uri.parse("$baseUrl/$path"),
        body: json.encode(body), headers: {"Content-Type": "application/json"});
    if (res.statusCode == 200) {
      return json.decode(utf8.decode(res.bodyBytes));
    } else {
      _throwHttpError(res);
    }
  }

  Future<void> delete(String path) async {
    var res = await client.delete(Uri.parse("$baseUrl/$path"));
    if (res.statusCode == 204) {
      return;
    } else {
      _throwHttpError(res);
    }
  }

  void _throwHttpError(Response res) {
    throw HttpException("HttpError: ${res.statusCode}\n${res.body}");
  }
}
