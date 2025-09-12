# building process
FROM nginx:latest
WORKDIR /usr/src/app
run apt-get update &&\
  apt-get install -y git curl &&\
  git clone https://github.com/huzzleonliu/wedding_project.git #实际用的时候改成仓库地址



# runtime process
FROM nginx:latest
copy /usr/src/app/print_project/dist/ /usr/share/nginx/html

RUN rm -rf /etc/nginx
COPY nginx /etc/nginx


CMD ["nginx", "-g", "daemon off;"]
