import { goto } from '$app/navigation';
import Cookies from 'js-cookie';

export async function beforeEnter(route) {
    const token = Cookies.get('token'); // Retrieve the token from the cookie
  
    if (!token) {
      // Token is missing, redirect to the login page
      goto('/login');
    }
    const backendUrl = import.meta.env.VITE_API_URL;
    const response = await fetch(backendUrl + '/validate-token', {
        method: 'Post',
        headers: {
            'Authorization' : `Bearer ${token}`,
        }
    });
    if (!response)
        goto ('/login');
    console.log('SUCCESS SOMEHOW IT FUCKING WORKS');
}
  
