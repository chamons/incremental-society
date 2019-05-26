window.SaveGame = (state) => {
	localStorage.setItem('save-state', state);
};

window.LoadGame = () => {
	var save = localStorage.getItem('save-state');
	if (save === null)
		return "";
	return save;
};

onBuildingModalHidden = function (event) { BuildingModalView.invokeMethod ('OnBuildingModalDismissed'); }
window.ShowBuildingModal = () => { $('#selectBuildingModal').modal('show'); };
window.DismissBuildingModal = () => { $('#selectBuildingModal').modal('hide'); };

onResearchModalHidden = function (event) { ResearchModalView.invokeMethod ('OnResearchModalDismissed'); }
window.ShowResearchModal = () => { $('#selectResearchModal').modal('show'); };
window.DismissResearchModal = () => { $('#selectResearchModal').modal('hide'); };

/*
 * These controls aren't created until C# renders tree, so we must delay this until then.
 * Also, we get this called after every render, so don't hook up controls that refresh.
 */
var BuildingModalView = null;
window.InitBuildingModal = (modalView) =>
{
	if (BuildingModalView === null) {
		BuildingModalView = modalView;
		$('#selectBuildingModal').on('hide.bs.modal', onBuildingModalHidden);
	}
}

var ResearchModalView = null;
window.InitResearchModal = (modalView) =>
{
	if (ResearchModalView === null) {
		ResearchModalView = modalView;
		$('#selectResearchModal').on('hide.bs.modal', onBuildingModalHidden);
	}
}
