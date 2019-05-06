window.ShowBuildingModal = () => {
	$('#selectBuildingModal').modal('show');
};

window.DismissBuildingModal = () => {
	$('#selectBuildingModal').modal('hide');
};

onBuildingModalShown = function (event) {
}

onBuildingModalHidden = function (event) {
	ModalView.invokeMethod ('OnModalDismissed');
}

/* 
 * These controls aren't created until C# renders tree, so we must delay this until then.
 * Also, we get this called after every render, so don't hook up controls that refresh.
 */
var ModalView = null;
window.InitBuildingModal = (modalView) =>  
{
	if (ModalView === null) {
		ModalView = modalView;
		$('#selectBuildingModal').on('show.bs.modal', onBuildingModalShown);
		$('#selectBuildingModal').on('hide.bs.modal', onBuildingModalHidden);
	}
}
