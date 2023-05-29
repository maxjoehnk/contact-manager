import 'package:flutter/material.dart';

class ContactFormField extends StatelessWidget {
  final TextEditingController controller;
  final String label;
  final Function()? submit;
  final String? Function(String?)? validator;
  final bool autofocus;
  final bool required;

  const ContactFormField(
      {Key? key,
      required this.controller,
      required this.label,
      this.validator,
      this.submit,
      this.autofocus = false,
      this.required = false})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: TextFormField(
          controller: controller,
          autofocus: autofocus,
          decoration: InputDecoration(
            filled: true,
            border: const OutlineInputBorder(),
            labelText: required ? "$label*" : label,
          ),
          onFieldSubmitted: submit == null ? null : (_) => submit!(),
          validator: validator,
        ),
      ),
    );
  }
}
