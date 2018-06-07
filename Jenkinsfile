podTemplate(label: 'buildPod', containers: [
  containerTemplate(name: 'docker', image: 'docker', ttyEnabled: true, command: 'cat')  
],
volumes: [
  hostPathVolume(mountPath: '/var/run/docker.sock', hostPath: '/var/run/docker.sock'),
]) {
  node('buildPod') {
    def repo = checkout scm
    def gitCommit = repo.GIT_COMMIT
    def gitBranch = repo.GIT_BRANCH
    def shortGitCommit = "${gitCommit[0..10]}"

    stage('Build and Publish Docker image') {
      container('docker') {
        withCredentials([[$class: 'UsernamePasswordMultiBinding', credentialsId: 'dockerhub',
            usernameVariable: 'DOCKER_HUB_USER', passwordVariable: 'DOCKER_HUB_PASSWORD']]) {
              sh "docker build --rm -t=${env.DOCKER_HUB_USER}/rust-web ."
              sh "docker tag ${env.DOCKER_HUB_USER}/rust-web ${env.DOCKER_HUB_USER}/rust-web:${shortGitCommit}"
              sh "docker login -u ${env.DOCKER_HUB_USER} -p ${env.DOCKER_HUB_PASSWORD} "
              sh "docker push ${env.DOCKER_HUB_USER}/hmda-platform:${shortGitCommit}"
            }
        }
    }

    stage('Deploy') {
      container('helm') {
        sh "helm upgrade --install --force --namespace=default --values=helm-rust-web/values.yaml --set image.tag=${shortGitCommit} rust-web-${gitBranch} helm/rust-web"
      }
    }

    }
  }
}
