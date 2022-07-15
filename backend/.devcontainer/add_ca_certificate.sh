sudo sh -c "\
    mkdir /usr/share/ca-certificates/maimai/ && \
    curl https://jp.globalsign.com/repository/common/cer/gsrsaovsslca2018.cer > /usr/share/ca-certificates/maimai/gsrsaovsslca2018.cer && \
    echo 'maimai/gsrsaovsslca2018.cer' >> /etc/ca-certificates.conf && \
    update-ca-certificates"
