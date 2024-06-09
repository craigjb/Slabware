FROM --platform=linux/amd64 dockvado

RUN pip3 install fusesoc

COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

WORKDIR /home/user/Slabware
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD bash
