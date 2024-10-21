<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.014.23 Put Update Sub Module Penyesuaian</name>
   <tag></tag>
   <elementGuidId>41bd9902-cb8a-4519-a0a7-16dd8f7d8855</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;moduleId\&quot;: \&quot;6703b210e79c6e916327a42b\&quot;,\n    \&quot;name\&quot;: \&quot;Setting v2\&quot;,\n    \&quot;isIncludeFrontend\&quot;: false,\n    \&quot;isIncludeMobile\&quot;: false,\n    \&quot;isIncludeIot\&quot;: false,\n    \&quot;team\&quot;: {\n        \&quot;uiux\&quot;: \&quot;66e90198e64712c4e5eb0de9\&quot;,\n        \&quot;devOps\&quot;: \&quot;66d1771c1ebe5344bc2b9dab\&quot;,\n        \&quot;qcWeb\&quot;: \&quot;66ea825884cd207e408ae6ce\&quot;,\n        \&quot;qcMobile\&quot;: \&quot;66ea825884cd207e408ae6ce\&quot;\n    }\n}&quot;,
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
      <webElementGuid>81611cf3-2a6b-4e96-bdab-3ef987b8f94c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b65928a7-e553-4cb4-a676-3f4d6661b5a7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d9ce79e5-4312-40e3-9f4b-d33cf6ec9ab5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${submodul}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'66e901c165b533fbe0c72c03/project-detail/sub-module/66f12a9720a1e44d9a92c03d'</defaultValue>
      <description></description>
      <id>03838825-c9ba-4d03-b57f-958697be01df</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>8cfe06ac-c345-401f-b241-473a98d12bbc</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.submodul</defaultValue>
      <description></description>
      <id>ed0b6dc7-588c-49b4-b138-7a82c829812f</id>
      <masked>false</masked>
      <name>submodul</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
