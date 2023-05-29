import 'package:frontend/api/http.dart';

import 'models/distribution_list.dart';

class DistributionListApi {
  final HttpApi httpApi;

  DistributionListApi(this.httpApi);

  Future<List<DistributionList>> getDistributionLists() async {
    List<dynamic> res = await httpApi.get("v1/distributionLists");

    return res.map((e) => DistributionList.fromJson(e)).toList();
  }

  Future<String> createDistributionList(
      CreateDistributionList distributionList) async {
    return await httpApi.post(
        "v1/distributionLists", distributionList.toJson());
  }
}
