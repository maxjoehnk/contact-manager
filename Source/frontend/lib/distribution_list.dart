import 'package:flutter/material.dart';
import 'package:frontend/state/distribution_list_state.dart';
import 'package:provider/provider.dart';

class DistributionListView extends StatelessWidget {
  const DistributionListView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<DistributionListState>(
      builder: (context, state, child) {
        if (state.loading) {
          return const Center(child: CircularProgressIndicator());
        }

        if (state.distributionLists.isEmpty) {
          return Center(
              child: Text("Es gibt noch keine Verteilerlisten",
                  style: Theme.of(context).textTheme.bodyMedium));
        }

        return ListView.builder(
          itemCount: state.distributionLists.length,
          itemBuilder: (context, index) {
            var distributionList = state.distributionLists[index];
            return ListTile(
              selected: state.selected == distributionList,
              selectedTileColor: Theme.of(context).colorScheme.primaryContainer,
              title: Text(distributionList.name),
              subtitle: Text("${distributionList.contacts.length} Kontakte"),
              onTap: () => state.select(distributionList),
            );
          },
        );
      },
    );
  }
}
