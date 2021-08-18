<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Verify Acquisition</name>
   <tag></tag>
   <elementGuidId>66d9d4f6-263a-4777-a124-0fb5715be985</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{        \r\n        \&quot;merchant_table_amount\&quot;: 13,\r\n        \&quot;acquisition_form_address\&quot;: \&quot;Jalan Harapan No.3 Malayka Resident No.5 RT.3/RW.3, Bintaro Kec. Pesanggrahan Kota Jakarta Selatan Daerah Khusus Ibukota Jakarta Kode Pos 12330 (021) 29866319\&quot;,\r\n        \&quot;acquisition_form_detail\&quot;: \&quot;\&quot;,\r\n        \&quot;acquisition_form_subscription\&quot;: \&quot;35ec44bd-8d3b-4e3a-a217-4002b8ad1c49\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BO-Silvi-Staging}/backoffice/acquisition/${acquisitionId}/verify</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BO-Silvi-Staging</defaultValue>
      <description></description>
      <id>b68e2418-5d61-4854-900a-f1e3af8a2de5</id>
      <masked>false</masked>
      <name>BO-Silvi-Staging</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.acquisitionId</defaultValue>
      <description></description>
      <id>32bb0fdb-25bb-4150-ab2c-8dd04b93e385</id>
      <masked>false</masked>
      <name>acquisitionId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
