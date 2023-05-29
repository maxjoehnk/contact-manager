import 'package:flutter/material.dart';
import 'package:frontend/api/models/distribution_list.dart';
import 'package:frontend/state/distribution_list_state.dart';
import 'package:frontend/widgets/form_field.dart';
import 'package:provider/provider.dart';

class EditDistributionListDialog extends StatefulWidget {
  final DistributionList? distributionList;

  const EditDistributionListDialog({Key? key, this.distributionList})
      : super(key: key);

  @override
  State<EditDistributionListDialog> createState() =>
      _EditDistributionListDialogState();
}

class _EditDistributionListDialogState
    extends State<EditDistributionListDialog> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();

  @override
  void initState() {
    super.initState();
    if (widget.distributionList != null) {
      _nameController.text = widget.distributionList!.name;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: Form(
        key: _formKey,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400, maxHeight: 600),
          child: Container(
            padding: const EdgeInsets.all(16),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text("Verteiler hinzufügen",
                    style: Theme.of(context).textTheme.titleLarge),
                Row(children: [
                  ContactFormField(
                    controller: _nameController,
                    label: "Name",
                    autofocus: true,
                    required: true,
                    submit: _submit,
                    validator: (value) {
                      if (value == null || value.isEmpty) {
                        return 'Bitte geben Sie einen Namen ein.';
                      }
                      return null;
                    },
                  )
                ]),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    ElevatedButton(
                        onPressed: _submit, child: const Text("Speichern"))
                  ],
                )
              ],
            ),
          ),
        ),
      ),
    );
  }

  void _submit() {
    if (_formKey.currentState!.validate()) {
      _save();
    }
  }

  void _save() async {
    var state = Provider.of<DistributionListState>(context, listen: false);
    await state.createDistributionList(_assembleDistributionList());

    Navigator.of(context).pop();
  }

  CreateDistributionList _assembleDistributionList() {
    return CreateDistributionList(
      name: _nameController.text,
      contacts: [],
    );
  }
}
