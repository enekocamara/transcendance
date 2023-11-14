$("#registerUser").submit(function(event) {
    event.preventDefault(); // Prevent the default form submission

    var username = $("#usernameInput").val();
    var password1 = $("#passwordInput").val();
	var password2 = $("#repeatPasswordInput").val();

    if(password1 !== password2) {
        document.getElementById("registrationBad").innerHTML = '<i class="bi bi-x-circle-fill"></i> Passwords do not match!';
        document.getElementById("registrationBad").classList.remove("d-none");

		return false;
    }
	var apiUrl = "http://10.13.8.2:5000/members/register/";

    var data = {
        "username": username,
        "password1": password1,
        "password2": password2
    };

    $.ajax({
        type: "POST",
        url: apiUrl,
        data: JSON.stringify(data),
        dataType: "json",
        contentType: "application/json",
        success: function(response) {
            setTimeout(function() {
                $("#registrationGood").removeClass("d-none");
                $("#registrationGood").slideDown('slow');
            }, 5000);
            window.location.href = "http://10.13.8.2";
            //window.location.href = process.env.API_URL;
        },

        error: function(error) {
			if (error.responseJSON && error.responseJSON.message) {
                $('#registrationBad').html('<i class="bi bi-x-circle-fill"></i>     ' + error.responseJSON.message).slideDown('slow');
                $('#registrationBad').removeClass('d-none');
                $('#registrationBad').slideDown('slow');

                setTimeout(function() {
                    $('#registrationBad').slideUp('slow', function() {
                    $('#registrationBad').addClass('d-none');
                    });
                }, 5000);
			} else {
				$("#registrationBad").html('<i class="bi bi-x-circle-fill"></i> An unknown error ocurred when trying to do the registration!');
                $('#registrationBad').removeClass('d-none');
                $('#registrationBad').slideDown('slow');

                setTimeout(function() {
                    $('#registrationBad').slideUp('slow', function() {
                    $('#registrationBad').addClass('d-none');
                    });
                }, 5000);
			}
        }
    });
});

$('#cuarta').change(function() {
    var selectedValue = $(this).val();
    if (selectedValue === "5") {
        $('#inputAmigo').slideDown('slow');
    } else {
        $('#inputAmigo').slideUp('slow');
    }
	
	if (selectedValue === "6") {
		$('#inputOtro').slideDown('slow');
	} else {
		$('#inputOtro').slideUp('slow');
	}
});