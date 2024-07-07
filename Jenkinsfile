pipeline {
    agent any

    environment {
        REPO_URL = 'https://github.com/kirellkekw/cu-yemek-rust.git'
        IMAGE_NAME = 'cu-yemek-rust'
    }

    stages {
        stage('Checkout') {
            steps {
                git url: "${env.REPO_URL}", branch: 'main'
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    def image = docker.build("${env.IMAGE_NAME}:${env.BUILD_NUMBER}")
                }
            }
        }

        stage('Deploy') {
            steps {
                script {
                    // Stop and remove the existing container if it exists
                    sh '''
                    if [ $(docker ps -a -q --filter name=cu-yemek-rust) ]; then
                        docker stop cu-yemek-rust
                        docker rm cu-yemek-rust
                    fi
                    '''

                    // Run the new container
                    sh '''
                    docker run -d -p 31000:2002 --name cu-yemek-rust --restart always ${env.IMAGE_NAME}:${env.BUILD_NUMBER} 
                    '''
                }
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            echo 'Build and deployment succeeded!'
        }
        failure {
            echo 'Build or deployment failed!'
        }
    }
}
