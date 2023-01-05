import '../App.css';
import NavigationComponent from "../component/Navigation";
import PostSendFormComponent from "../component/PostSendForm";


function CreatePost() {

    console.log(localStorage.getItem("user-token"));

    return (
        <div className="App">
            <NavigationComponent/>
            <PostSendFormComponent/>
        </div>
    );
}

export default CreatePost;