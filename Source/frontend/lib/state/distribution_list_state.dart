import 'dart:developer';

import 'package:flutter/cupertino.dart';
import 'package:frontend/api/distribution_list_api.dart';
import 'package:frontend/api/models/distribution_list.dart';

class DistributionListState extends ChangeNotifier {
  final DistributionListApi _api;
  List<DistributionList> _distributionLists = [];
  DistributionList? _selectedDistributionList;
  bool _loading = false;

  DistributionListState(this._api) {
    refresh();
  }

  List<DistributionList> get distributionLists => _distributionLists;

  DistributionList? get selected => _selectedDistributionList;

  bool get loading => _loading;

  Future<void> refresh() async {
    _loading = true;
    notifyListeners();

    try {
      _distributionLists = await _api.getDistributionLists();
    } catch (e) {
      log(e.toString());
    } finally {
      _loading = false;
    }
    notifyListeners();
  }

  Future<void> createDistributionList(
      CreateDistributionList distributionList) async {
    await _api.createDistributionList(distributionList);
    await refresh();
  }

  select(DistributionList? distributionList) {
    _selectedDistributionList = distributionList;
    notifyListeners();
  }
}
