# Junior club backend


* actix-web version - 4.2.1
* diesel version - 2.0.2
* DB postgres:14.5


# Build dev version:

 
All commands in the root directory -  cd ../backend

* #### If you don't have postgres (for Windows), download it here:

    -  https://www.postgresql.org/download/windows/

    -  add the bin/ directory of PostgreSQL to your PATH environment variable.


###### More information on installing postgres as well as installing on other OC:

#### https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md


* #### Use the DATABASE_URL variable marked dev in the [.env file](.env)


* #### After installing postgres and defining the environment variable, create a container in the docker for the database

    
    docker-compose up

* #### Install a diesel client

    
    cargo install diesel_cli --no-default-features --features postgres


* #### Setting DB:

    
    diesel setup

###### If you have problems or need more information about installing and creating migrations:

#### https://diesel.rs/guides/getting-started

* #### And finally, you can run migration

    
    diesel migration run

* #### Run server


    cargo run









