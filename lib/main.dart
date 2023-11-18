import 'package:flutter/material.dart';
import 'dart:async'; 
import 'login.dart';

void main() => runApp(MaterialApp( 
      debugShowCheckedModeBanner: false, 
      initialRoute: '/', 
      routes: {'/': (context) => Login()}, 
));