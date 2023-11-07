$("#registerUser").submit(function(event) {
    event.preventDefault(); // Prevent the default form submission

    var username = $("#usernameInput").val();
    var password = $("#passwordInput").val();
	var repeatPassword = $("#repeatPasswordInput").val();

    if(password !== repeatPassword) {
        document.getElementById("registrationBad").innerHTML = '<i class="bi bi-x-circle-fill"></i> Passwords do not match!';
        document.getElementById("registrationBad").classList.remove("d-none");

		return false;
    }

	var apiUrl = "http://10.13.8.2:5000/register";

    var data = {
        "username": username,
        "password": password
    };

    $.ajax({
        type: "POST",
        url: apiUrl,
        data: JSON.stringify(data),
        dataType: "json",
        contentType: "application/json",
        success: function(response) {
			$("#registrationGood").removeClass("d-none");
            console.log(response);
        },
        error: function(error) {
			if (error.responseJSON && error.responseJSON.message) {
				$("#registrationBad").removeClass("d-none");
				$("#registrationBad").html('<i class="bi bi-x-circle-fill"></i>' + error.responseJSON.message);
			} else {
				$("#registrationBad").removeClass("d-none");
				$("#registrationBad").html('<i class="bi bi-x-circle-fill"></i> An unknown error ocurred when trying to do the registration!');
			}
        }
    });
});
