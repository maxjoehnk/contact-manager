import 'package:flutter/material.dart';
import 'package:frontend/api/contact_api.dart';
import 'package:frontend/api/distribution_list_api.dart';
import 'package:frontend/api/http.dart';
import 'package:frontend/dialogs/edit_organization.dart';
import 'package:frontend/dialogs/edit_person.dart';
import 'package:frontend/state/distribution_list_state.dart';
import 'package:provider/provider.dart';

import 'contact_list.dart';
import 'dialogs/edit_distribution_list.dart';
import 'distribution_list.dart';
import 'state/contacts_state.dart';

void main() {
  runApp(const ContactManagerApp());
}

class ContactManagerApp extends StatelessWidget {
  const ContactManagerApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        Provider(create: (context) => HttpApi()),
        Provider(create: (context) => ContactApi(context.read())),
        Provider(create: (context) => DistributionListApi(context.read())),
        ChangeNotifierProvider(
            create: (context) => ContactsState(context.read())),
        ChangeNotifierProvider(
            create: (context) => DistributionListState(context.read())),
      ],
      child: MaterialApp(
        title: 'Contact Manager',
        theme: ThemeData.dark(useMaterial3: true),
        home: const HomeScreen(),
      ),
    );
  }
}

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    return Scaffold(
      body: Consumer<DistributionListState>(builder: (context, state, child) {
        return Row(
          children: [
            SizedBox(
                width: 400,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: ListTile(
                        title: const Text("Alle Kontakte"),
                        selected: state.selected == null,
                        onTap: () => state.select(null),
                        selectedTileColor: theme.colorScheme.primaryContainer,
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16.0),
                      child:
                          Text("Verteiler", style: theme.textTheme.titleLarge),
                    ),
                    const Expanded(
                        child: Padding(
                      padding: EdgeInsets.all(16.0),
                      child: DistributionListView(),
                    )),
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: TextButton(
                          onPressed: () {},
                          child: const Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(Icons.settings),
                              SizedBox(width: 8),
                              Text("Einstellungen"),
                            ],
                          )),
                    )
                  ],
                )),
            const VerticalDivider(
              width: 2,
              thickness: 2,
            ),
            Expanded(
              child: Padding(
                padding: const EdgeInsets.all(32.0),
                child: Column(children: [
                  QuickActions(distributionSelected: state.selected != null),
                  const SizedBox(height: 32),
                  const SearchBar(),
                  const SizedBox(height: 32),
                  Expanded(child: ContactListView(state.selected)),
                ]),
              ),
            ),
          ],
        );
      }),
    );
  }
}

class QuickActions extends StatelessWidget {
  final bool distributionSelected;

  const QuickActions({
    super.key,
    required this.distributionSelected,
  });

  @override
  Widget build(BuildContext context) {
    return Row(mainAxisAlignment: MainAxisAlignment.spaceAround, children: [
      QuickAction(
          title: "Kontakt hinzufügen",
          icon: Icons.account_circle,
          onTap: () {
            showDialog(
                context: context,
                builder: (context) => const EditPersonDialog());
          }),
      QuickAction(
          title: "Organisation hinzufügen",
          icon: Icons.add_business,
          onTap: () {
            showDialog(
                context: context,
                builder: (context) => const EditOrganizationDialog());
          }),
      if (!distributionSelected)
        QuickAction(
            title: "Verteiler hinzufügen",
            icon: Icons.groups,
            onTap: () {
              showDialog(
                  context: context,
                  builder: (context) => const EditDistributionListDialog());
            }),
    ]);
  }
}

class QuickAction extends StatelessWidget {
  final String title;
  final IconData icon;
  final Function() onTap;

  const QuickAction(
      {required this.title, required this.icon, required this.onTap, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    return Container(
      height: 96,
      width: 256,
      decoration: BoxDecoration(
        borderRadius: const BorderRadius.all(Radius.circular(8)),
        color: theme.colorScheme.primaryContainer,
      ),
      child: InkWell(
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                Icon(icon),
                Text(title, style: theme.textTheme.titleMedium),
              ]),
        ),
      ),
    );
  }
}

class SearchBar extends StatelessWidget {
  const SearchBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      decoration: InputDecoration(
        filled: true,
        hintText: "Suche",
        prefixIcon: const Icon(Icons.search),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(8),
        ),
      ),
    );
  }
}
