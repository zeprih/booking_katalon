<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>createBooking</name>
   <tag></tag>
   <elementGuidId>6513fa2e-39ca-4c7f-91d5-4bf3bc38374a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstname}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${lastname}\&quot;,\n    \&quot;totalprice\&quot; : ${totalprice},\n    \&quot;depositpaid\&quot; : ${depositpaid},\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;${checkin}\&quot;,\n        \&quot;checkout\&quot; : \&quot;${checkout}\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${additionalneeds}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Jhon'</defaultValue>
      <description></description>
      <id>374205ab-0f11-40fe-9b23-fec19686981b</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'David'</defaultValue>
      <description></description>
      <id>258de8bf-648c-4593-8af4-49508840ddf0</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>111</defaultValue>
      <description></description>
      <id>7d9a23bd-2173-4e69-8625-ac2627ad13c0</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>9e6f36e0-41a5-48b0-89d7-878bcffab852</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2018-01-01'</defaultValue>
      <description></description>
      <id>f1e8712a-ccd7-4c14-818f-6f8f38f72b08</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2019-01-01'</defaultValue>
      <description></description>
      <id>fc046401-401f-4c2f-815f-39423fdcad78</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'Breakfast'</defaultValue>
      <description></description>
      <id>684bc3c4-0eac-4455-b73c-b520ff5f5c81</id>
      <masked>false</masked>
      <name>additionalneeds</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
